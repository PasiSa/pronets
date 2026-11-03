---
title: Project goals and evaluation criteria
lang: en
translation_key: project
---

Alongside with the assignments, a small networked software project is developed
during the course, consisting of components developed during the assignments.
The project is graded at a point scale between 0 - 6 points, which are added to
the assignment points (7 assignment rounds, total max. 21 points).

The project goals and evaluation criteria are composed of following categories,
and relative weights in evaluation:

## Documentation (2)

The document is preferably done in the git repository as a **markdown file**,
for example extending the `design.md` started earlier. The document should cover
the following parts:

- Scope of the project and implemented features. Main functionality described
- Description of the program structure: what modules and `.rs` sources is the
  project composed of.
- Libraries and crates used, along with short description of why they are
  needed.
- Approach to handling concurrent clients and concurrent input sources at the
  server and the client.
- Specific **description of protocol messages** used in the project (important)
- Description of testing practices applied: what kind of unit tests, integration
  tests, or some other types of tests were applied.
- Development tools and practices used: description of the programming
  environment, division of work if the project was done with a pair, and
  description of what AI tools were used and how they were applied in the
  development cycle.
- Instructions for running and testing the implementation
- Reflection of the lessons learned during the project, and the known
  shortcomings of the final result. For example, you can describe what could
  have been done differently, if there was another attempt or more time.
- Your subjective own evaluation how well did the project succeed in following
  the criteria described here.

## Code clarity and structure (1)

- The code should be distributed into logical and easily maintainable functions.
- The code should be distributed into logical modules, everything should not be
  in `main.rs`.
- Formatting and naming conventions of functions and types should be clear and
  consistent. Note that rust has formatting tool that can help in this. You can
  run it, for example, using cargo: `cargp fmt`.

## Functionality (2)

- The should no major bugs that prevent normal use and testimg of the client and
  server implementations.
- Server should run robustly, even if there are errors and exceptional
  situations. The error handling should be appropriate and not cause
  interruption of the server execution in runtime panic.
- Server should handle multiple simultaneous clients in a timely way without too
  much delays.
- There should be unit and integration tests, especially on the key server
  functionality.
- Tracing and logging should be sufficient and logically structure, e.g. in
  different logging levels.

## Communication features (1)

- TLS should be supprted for TCP connections. The projects do not need to use
  TLS with UDP.
- UDP protocol is used in some reasonable form in the project
- The client and server implementations interoperate (and tested) with at least
  one other, independent implementation of the same topic. Interoperability does
  not need to cover all features, but at least some basic topic-specific
  features (basic objects on a whiteboard, or position updates om a car game).

The evaluation is done in a holisitic way along the above categories: if the
project falls short on some of the mentioned criteria, it is possible to
compensate by doing a comprehensive work on others.
