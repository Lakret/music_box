use actix_web_static_files::NpmBuild;

fn main() {
  NpmBuild::new("./assets")
    .executable("parcel")
    .install()
    .unwrap()
    .run("build")
    .unwrap()
    .target("./assets/dist")
    .to_resource_dir()
    .build()
    .unwrap();
}
