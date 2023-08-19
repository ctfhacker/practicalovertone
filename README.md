# Soundgen

Proof of concept application for generating sounds.

* application - `egui` powered GUI
* practicalovertone - The library used to generate/process audio signals
* `app_state` - The data state used by both the GUI and library
  - This is seperated to enable "hot reloading" of new library code

# Starting

#### One terminal

```
cargo run -r
```

```
cargo watch -x 'build -r'

# Change the code in practicalovertone
# Will reload on save to the running application
```

