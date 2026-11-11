use sastra_core::{
    ChangeKind, Error, FORMAT_VERSION, NewProduct, ProductPatch, ProductStatus, Project,
};

fn camiseta() -> NewProduct {
    NewProduct {
        code: "SS27-TS-001".into(),
        name: "Camiseta básica".into(),
        season: "SS27".into(),
        collection: "Esenciales".into(),
        garment_type: "Camiseta".into(),
    }
}

#[test]
fn crear_y_reabrir_conserva_los_datos() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("coleccion.sastra");

    let id = {
        let mut project = Project::create(&path).unwrap();
        project.create_product(camiseta()).unwrap().id
    };

    let project = Project::open(&path).unwrap();
    let products = project.list_products().unwrap();
    assert_eq!(products.len(), 1);
    assert_eq!(products[0].id, id);
    assert_eq!(products[0].name, "Camiseta básica");
    assert_eq!(products[0].status, ProductStatus::Draft);
    assert!(project.meta("created_at").unwrap().is_some());
}

#[test]
fn no_sobrescribe_un_archivo_existente() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("existe.sastra");
    std::fs::write(&path, b"algo").unwrap();
    assert!(matches!(
        Project::create(&path),
        Err(Error::AlreadyExists(_))
    ));
}

#[test]
fn rechaza_archivos_que_no_son_de_sastra() {
    let dir = tempfile::tempdir().unwrap();

    let texto = dir.path().join("texto.sastra");
    std::fs::write(&texto, "esto no es una base de datos").unwrap();
    assert!(matches!(
        Project::open(&texto),
        Err(Error::NotASastraFile(_))
    ));

    // Una base SQLite de otra aplicación tampoco vale.
    let ajena = dir.path().join("ajena.sastra");
    rusqlite::Connection::open(&ajena)
        .unwrap()
        .execute_batch("CREATE TABLE t (x INTEGER);")
        .unwrap();
    assert!(matches!(
        Project::open(&ajena),
        Err(Error::NotASastraFile(_))
    ));
}

#[test]
fn rechaza_formatos_mas_nuevos() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("futuro.sastra");
    drop(Project::create(&path).unwrap());
    rusqlite::Connection::open(&path)
        .unwrap()
        .pragma_update(None, "user_version", FORMAT_VERSION + 1)
        .unwrap();
    assert!(matches!(
        Project::open(&path),
        Err(Error::NewerFormat { .. })
    ));
}

#[test]
fn el_codigo_es_obligatorio_y_unico_sin_distinguir_mayusculas() {
    let dir = tempfile::tempdir().unwrap();
    let mut project = Project::create(dir.path().join("p.sastra")).unwrap();
    project.create_product(camiseta()).unwrap();

    let mut repetido = camiseta();
    repetido.code = "ss27-ts-001".into();
    assert!(matches!(
        project.create_product(repetido),
        Err(Error::DuplicateCode(_))
    ));

    let mut sin_codigo = camiseta();
    sin_codigo.code = "   ".into();
    assert!(matches!(
        project.create_product(sin_codigo),
        Err(Error::Invalid(_))
    ));
}

#[test]
fn actualizar_solo_cambia_lo_indicado_y_queda_registrado() {
    let dir = tempfile::tempdir().unwrap();
    let mut project = Project::create(dir.path().join("p.sastra")).unwrap();
    let original = project.create_product(camiseta()).unwrap();

    let updated = project
        .update_product(
            &original.id,
            ProductPatch {
                name: Some("Camiseta básica algodón orgánico".into()),
                status: Some(ProductStatus::InDevelopment),
                ..Default::default()
            },
        )
        .unwrap();

    assert_eq!(updated.code, original.code);
    assert_eq!(updated.season, original.season);
    assert_eq!(updated.name, "Camiseta básica algodón orgánico");
    assert_eq!(updated.status, ProductStatus::InDevelopment);
    assert!(updated.updated_at > original.updated_at);

    let changes = project.recent_changes(10).unwrap();
    assert_eq!(changes.len(), 2);
    assert_eq!(changes[0].kind, ChangeKind::Update);
    assert_eq!(
        changes[0].before.as_ref().unwrap()["name"],
        "Camiseta básica"
    );
    assert_eq!(
        changes[0].after.as_ref().unwrap()["name"],
        "Camiseta básica algodón orgánico"
    );
    assert_eq!(changes[1].kind, ChangeKind::Create);
}

#[test]
fn un_cambio_vacio_no_genera_registro() {
    let dir = tempfile::tempdir().unwrap();
    let mut project = Project::create(dir.path().join("p.sastra")).unwrap();
    let p = project.create_product(camiseta()).unwrap();
    project
        .update_product(
            &p.id,
            ProductPatch {
                name: Some(p.name.clone()),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(project.recent_changes(10).unwrap().len(), 1);
}

#[test]
fn borrar_guarda_como_estaba() {
    let dir = tempfile::tempdir().unwrap();
    let mut project = Project::create(dir.path().join("p.sastra")).unwrap();
    let p = project.create_product(camiseta()).unwrap();
    project.delete_product(&p.id).unwrap();

    assert!(project.list_products().unwrap().is_empty());
    assert!(matches!(
        project.get_product(&p.id),
        Err(Error::ProductNotFound(_))
    ));
    let last = &project.recent_changes(1).unwrap()[0];
    assert_eq!(last.kind, ChangeKind::Delete);
    assert_eq!(last.before.as_ref().unwrap()["code"], "SS27-TS-001");
}

#[test]
fn la_copia_se_abre_como_proyecto_independiente() {
    let dir = tempfile::tempdir().unwrap();
    let mut project = Project::create(dir.path().join("p.sastra")).unwrap();
    project.create_product(camiseta()).unwrap();

    let copy_path = dir.path().join("copia.sastra");
    project.save_copy_to(&copy_path).unwrap();
    assert!(matches!(
        project.save_copy_to(&copy_path),
        Err(Error::AlreadyExists(_))
    ));

    let copy = Project::open(&copy_path).unwrap();
    assert_eq!(copy.list_products().unwrap().len(), 1);
}

#[test]
fn el_proyecto_es_un_unico_archivo() {
    let dir = tempfile::tempdir().unwrap();
    let mut project = Project::create(dir.path().join("p.sastra")).unwrap();
    project.create_product(camiseta()).unwrap();
    drop(project);
    let files: Vec<_> = std::fs::read_dir(dir.path()).unwrap().collect();
    assert_eq!(
        files.len(),
        1,
        "no debe quedar ningún archivo auxiliar junto al proyecto"
    );
}
