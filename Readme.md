# Rust practice #

### Initial step ##
Step 1 - new project 
````
cargo new {project_name}
````
Optional - if error `` Blocking waiting for file lock on the registry index``
````
rm -rf ~/.cargo/registry/index/* ~/.cargo/.package-cache

or

cargo clean
````