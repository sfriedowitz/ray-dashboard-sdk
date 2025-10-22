use once_cell::sync::OnceCell;
use testcontainers::GenericImage;

static BASE: OnceCell<String> = OnceCell::new();
