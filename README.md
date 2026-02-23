# State Machines in Rust – A Game Dev Perspective

This repository compares five different approaches to implementing state machines in Rust, motivated by practical game development needs. The examples are intentionally minimal (two states toggling back and forth) to keep the focus on the structural differences.

## Context

In game development, state machines typically fall into two categories:

**High-level game states** (Menu → Game → Pause → Game Over → ...): These switch relatively rarely (on user interaction), but individual states can be heavyweight — a level state might hold large tilemaps, pathfinding grids, or precomputed lookup tables. Destroying and rebuilding these on every transition is wasteful. On consoles, predictable memory usage is also a concern: ideally, all states are allocated upfront so the memory footprint is known at startup.

**Small, object-local state machines** (Idle → Patrol → Chase → Attack): Hundreds of game entities may each carry one of these. They switch frequently (potentially every frame), hold minimal data, and need to be cache-friendly, cloneable, and cheap. Abstraction overhead (vtables, heap allocation) isn't worth it here.

## The Five Approaches

### `old_fashioned` – Enum + Match ⭐

The simplest approach: a single struct holds all state data, and `update` uses `match` on an enum index. No traits, no heap, no indirection.

**Best for:** Small, object-local state machines where simplicity and performance matter more than encapsulation. This is the go-to for per-entity states in game objects.

### `with_trait_objects` – Box\<dyn Trait\> + BlackBoard

The textbook OOP-style approach. Each state is a separate struct implementing a `StateControl` trait. The active state is held as `Box<dyn StateControl>`. On transition, the old state is destroyed and a new one is heap-allocated. A `BlackBoard` struct is used to preserve data across transitions (since the state objects themselves are ephemeral).

**Best for:** Situations where the set of states isn't known at compile time, or where you want maximum decoupling. The downside is that states are destroyed and recreated on every transition — expensive if states hold large data.

### `with_enum_dispatch` – enum\_dispatch + BlackBoard

Structurally identical to `with_trait_objects`, but replaces dynamic dispatch with the [`enum_dispatch`](https://crates.io/crates/enum_dispatch) crate. The trait calls are resolved at compile time via a generated `match`, eliminating vtable overhead. States are still destroyed and recreated on transition, so the BlackBoard is still needed.

**Best for:** A drop-in optimization over trait objects when the set of states is known at compile time and you want to keep the trait-based structure.

### `with_enum_map_trait_objects` – Persistent States + Trait Objects ⭐

All states are created upfront and stored in an [`EnumMap`](https://crates.io/crates/enum_map) indexed by a `StateIndex` enum. Only the index switches on transition — no state is ever destroyed or recreated. Uses trait objects (`Box<dyn StateControl>`) for the individual states.

**This is the author's preferred approach for high-level game states.** The memory footprint is predictable (all states live for the program's lifetime), expensive states like levels aren't rebuilt on transition, and the trait object overhead is irrelevant at this switching frequency. The `BlackBoard` shifts its role from state *preservation* (no longer needed) to inter-state *communication*.

### `with_enum_map_dispatch` – Persistent States + enum\_dispatch

Combines both optimizations: persistent states via `EnumMap` and static dispatch via `enum_dispatch`. Maximum performance, but also maximum crate dependencies and complexity for what is typically a non-bottleneck.

**Best for:** The theoretical optimum if you need both persistent states and high-frequency switching (e.g., hundreds of AI agents with heavyweight states). In practice, this combination is rarely necessary.

## Summary

| Approach | Dispatch | States persist | Heap alloc on transition | Crate deps | Recommended use case |
|---|---|---|--------------------------|---|---|
| `old_fashioned` | match | inherent | none                      | none | Per-entity states |
| `with_trait_objects` | dynamic | no | yes                      | none | Textbook / open state sets |
| `with_enum_dispatch` | static | no | none                     | `enum_dispatch` | Perf-sensitive, closed state set |
| `with_enum_map_trait_objects` | dynamic | yes | none                     | `enum_map` | **High-level game states** |
| `with_enum_map_dispatch` | static | yes | none                     | both | Theoretical optimum |

## Practical Recommendation

For most games, two approaches cover all needs:

- **`old_fashioned`** for the many small state machines on individual game objects
- **`enum_map_trait_objects`** for the few large, high-level game states

The other variants exist on the spectrum and have their place, but these two represent the practical sweet spots where the trade-offs align with actual game development requirements.

## Dependencies

```toml
[dependencies]
enum_dispatch = "0.3"
enum_map = "2.7"
```

Note: `is_multiple_of` requires Rust nightly or 1.84+. Replace with `x % n == 0` for older compilers.

## Discussion

I'm developing these examples for a Rust course focused on game programming. I'd be curious to hear:

- Do these categories match your experience?
- Are there other patterns you'd reach for?
- Any concerns about the `enum_map` + trait objects combination?

Feedback welcome!
