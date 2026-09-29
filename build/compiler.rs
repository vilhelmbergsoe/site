use std::{error::Error, fs, path::PathBuf};

use typst::{
    diag::{FileError, FileResult, SourceDiagnostic},
    foundations::{Bytes, Datetime},
    syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot},
    text::{Font, FontBook},
    utils::LazyHash,
    Feature, Library, LibraryExt, World,
};
use typst_kit::{
    diagnostics::{
        emit, termcolor::ColorChoice, termcolor::StandardStream, DiagnosticFormat, DiagnosticWorld,
    },
    files::{FileLoader, FileStore},
};

pub struct Compiler {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    files: FileStore<SiteFiles>,
}

struct SiteFiles {
    project_root: PathBuf,
    package_root: PathBuf,
}

pub struct SiteWorld<'a> {
    compiler: &'a Compiler,
    main: FileId,
    source: Source,
    path: PathBuf,
}

impl Compiler {
    pub fn new(project_root: PathBuf) -> Self {
        assert!(project_root.is_absolute());
        assert!(project_root.is_dir());

        let fonts: Vec<Font> = typst_assets::fonts()
            .flat_map(|data| Font::iter(Bytes::new(data)))
            .collect();
        let book = FontBook::from_fonts(&fonts);
        let files = FileStore::new(SiteFiles::new(project_root));

        assert!(!fonts.is_empty());
        assert!(book.contains_family("new computer modern math"));
        Self {
            library: LazyHash::new(
                Library::builder()
                    .with_features([Feature::Html].into_iter().collect())
                    .build(),
            ),
            book: LazyHash::new(book),
            fonts,
            files,
        }
    }

    pub fn world<'a>(&'a self, path: &str, text: String) -> SiteWorld<'a> {
        assert!(!path.is_empty());
        assert!(!text.is_empty());

        let virtual_path = VirtualPath::new(path).expect("the source path is valid");
        let main = FileId::new(RootedPath::new(VirtualRoot::Project, virtual_path));
        let source = Source::new(main, text);

        assert_eq!(source.id(), main);
        assert!(!self.fonts.is_empty());
        SiteWorld {
            compiler: self,
            main,
            source,
            path: PathBuf::from(path),
        }
    }

    pub fn math_font(&self) -> Option<&Font> {
        assert!(!self.fonts.is_empty());
        assert!(self.book.contains_family("new computer modern math"));

        let font = self
            .fonts
            .iter()
            .find(|font| font.post_script_name().as_deref() == Some("NewCMMath-Regular"));

        if let Some(font) = font {
            assert_eq!(font.info().family, "New Computer Modern Math");
        }
        font
    }
}

impl SiteFiles {
    fn new(project_root: PathBuf) -> Self {
        assert!(project_root.is_absolute());
        assert!(project_root.is_dir());

        let package_root = project_root.join("typst/packages");
        assert!(package_root.is_absolute());
        assert!(package_root.is_dir());
        Self {
            project_root,
            package_root,
        }
    }

    fn resolve(&self, id: FileId) -> FileResult<PathBuf> {
        assert!(self.project_root.is_absolute());
        assert!(self.package_root.is_absolute());

        let root = match id.root() {
            VirtualRoot::Project => self.project_root.clone(),
            VirtualRoot::Package(package) => self
                .package_root
                .join(package.namespace.as_str())
                .join(package.name.as_str())
                .join(package.version.to_string()),
        };
        let path = id.vpath().realize(&root)?;

        assert!(path.is_absolute());
        assert!(path.starts_with(&root));
        Ok(path)
    }
}

impl FileLoader for SiteFiles {
    fn load(&self, id: FileId) -> FileResult<Bytes> {
        assert!(self.project_root.is_dir());
        assert!(self.package_root.is_dir());

        let path = self.resolve(id)?;
        let data = fs::read(&path).map_err(|error| FileError::from_io(error, &path))?;

        assert!(path.is_file());
        assert_eq!(
            data.len() as u64,
            fs::metadata(&path)
                .map_err(|error| FileError::from_io(error, &path))?
                .len()
        );
        Ok(Bytes::new(data))
    }
}

pub fn emit_warnings(
    world: &SiteWorld<'_>,
    warnings: &[SourceDiagnostic],
) -> Result<(), Box<dyn Error>> {
    assert_eq!(world.source.id(), world.main);
    assert!(warnings.len() <= u32::MAX as usize);

    let warnings = warnings.iter().filter(|warning| {
        warning.message != "html export is under active development and incomplete"
    });
    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
    emit(&mut stderr, world, warnings, DiagnosticFormat::Human)?;

    assert_eq!(world.source.id(), world.main);
    assert!(world.path.is_relative());
    Ok(())
}

pub fn emit_diagnostics(
    world: &SiteWorld<'_>,
    diagnostics: &[SourceDiagnostic],
) -> Result<(), Box<dyn Error>> {
    assert_eq!(world.source.id(), world.main);
    assert!(!diagnostics.is_empty());

    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
    emit(
        &mut stderr,
        world,
        diagnostics.iter(),
        DiagnosticFormat::Human,
    )?;

    assert!(diagnostics
        .iter()
        .all(|diagnostic| !diagnostic.message.is_empty()));
    assert!(world.path.is_relative());
    Ok(())
}

impl DiagnosticWorld for SiteWorld<'_> {
    fn name(&self, id: FileId) -> String {
        assert!(!id.vpath().get_without_slash().is_empty());
        if id == self.main {
            assert_eq!(id.vpath(), self.main.vpath());
        }
        id.vpath().get_without_slash().to_owned()
    }
}

impl World for SiteWorld<'_> {
    fn library(&self) -> &LazyHash<Library> {
        assert!(!self.compiler.fonts.is_empty());
        assert_eq!(self.source.id(), self.main);
        &self.compiler.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        assert!(!self.compiler.fonts.is_empty());
        assert!(self
            .compiler
            .book
            .contains_family("new computer modern math"));
        &self.compiler.book
    }

    fn main(&self) -> FileId {
        assert!(!self.compiler.fonts.is_empty());
        assert_eq!(self.source.id(), self.main);
        self.main
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        assert!(!self.compiler.fonts.is_empty());
        assert_eq!(self.source.id(), self.main);
        if id == self.main {
            Ok(self.source.clone())
        } else {
            self.compiler.files.source(id)
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        assert!(!self.compiler.fonts.is_empty());
        assert_eq!(self.source.id(), self.main);
        self.compiler.files.file(id)
    }

    fn font(&self, index: usize) -> Option<Font> {
        assert!(!self.compiler.fonts.is_empty());
        assert!(self
            .compiler
            .book
            .contains_family("new computer modern math"));
        self.compiler.fonts.get(index).cloned()
    }

    fn today(&self, offset: Option<typst::foundations::Duration>) -> Option<Datetime> {
        assert!(!self.compiler.fonts.is_empty());
        assert_eq!(self.source.id(), self.main);
        let _ = offset;
        None
    }
}
