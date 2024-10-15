# Fox Object-Oriented Proto

A project designed to showcase an object-oriented approach for managing tile types. It defines key traits and impls for different tile types, such as HouseTile, enabling efficient interaction, mutability, and safe type casting. This system lays the groundwork for more complex structures, extending beyond tiles to other game objects in the future.

## Concept
This project is inspired by the object-oriented design principles found in [The Forgotten Server](https://github.com/otland/forgottenserver), particularly its handling of game entities like creatures, tiles, and containers. Similar to The Forgotten Server’s system for managing "things," this project explores how Rust’s traits can replace traditional object hierarchies, making the system both more flexible and performant.

By leveraging Rust’s zero-cost abstractions, minimizes runtime overhead while maintaining strict compile-time guarantees. It replaces the traditional use of object-oriented inheritance with traits.