# Scrape Collision Library

A wrapper around the rapier3d crate meant for specific use in Project: Scrape.

## Why

We don't want to use rapier calculations and dependencies in Zenitsu (the UDP server which is
responsible for the game logic).

Best examples for what type of abstraction we mean are the following 3 functions/structs:

### Game Collider
```rust
pub struct GameCollider {...}
```

which holds all the structs which rapier3d requires for calculations so that we can treat them as singletons.

### Load/Unload Player

```rust
pub fn load_player(&mut self, spawn: Vec<f32>) -> RigidBodyHandle { ... }
```

We wouldn't want to do these configurations in the game logic server since this is more-so maintanance and setup, rather than
actual logic which needs attention.


### Calculate Movement

```rust
pub fn calculate_movement(&mut self, handle: RigidBodyHandle, desired: Vec<f32>) -> [result] { ... }
```

We'd like to avoid the server logic doing mathematical conversions from vectors to matrices or isometries which 
rapier can process. So instead, we hide that complexity behind this method, as well as passing all the required parameters.

