# Scrape GLTF Loader

Re-write of Bevys GLTF loader meant for specific use for the Project Scrape.

## Why

We don't need all the complexity and edge cases covered in the original plugin. On top of that, we don't want to be 
dependent on the Bevy game engine given this is running on the server.

We only need the positions of the vertices, as well as the indices so we can re-create the trimesh on the server, which in turn,
would use that for collision-detection only. Rendering and such is done on Godot side.

Essentially, this is just a minified, by-itself version of the GLTF loader which Bevy uses.
