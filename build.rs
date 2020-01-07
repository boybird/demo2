use actix_web_static_files::resource_dir;

fn main() {

    resource_dir("./blog_front_end/build")

    .build().unwrap();
}
