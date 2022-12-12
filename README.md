# <abbr title="Abstract Syntax Tree">AST</abbr> Viewer UI

This project was inspired by [*"Zoom Out": The missing feature of IDEs*](https://medium.com/source-and-buggy/zoom-out-the-missing-feature-of-ides-f32d0f36f392).

I want to create a GUI with text fields, boxes, arrows, etc. and generates
source code, or even an AST that can be used by a compiler. So far I can take
Rust code and spit the AST back out.

## Roadmap

Subject to change of course, but here's the basic gist.

1. [ ] Round-trip any Rust file.
2. [ ] Have a minimal GUI.
3. [ ] Allow for editing AST nodes.
4. [ ] Output a new source file after editing the AST.
5. [ ] Allow for editing files in a whole Rust project.
7. [ ] Turn this into its own "language" as a thin wrapper over machine code, assembly, or
	 something like LLVM-IR, that allows for higher-level abstractions.
