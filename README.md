# BerryDelight

This project is being made as part of the Quantum Computational Chemistry Group that is within Quantum Computing at Berkeley

## About this project

BerryDelight is currently planned to be a simulation-first benchmark in quantum computational chemistry built around a question: when a variational quantum eigensolver is put up against a cheap classical surrogate, selected configuration interaction, does the quantum method actually win, and where?

There are no claims of quantum advantage being made for this project. The benchmark half is explicitly a learning substrate: build the full stack, understand the methods from the ground up, and reproduce a known result rigorously across molecules spanning weak single-reference correlation through strong multireference correlation, the regime where classical gold-standard methods begin to fail. Because the chosen systems will be small enough to have exact ground-truth energies, every method can be measured against the truth, and a negative result, where the surrogate matches or beats the quantum method, is a valid and expected outcome.

The second half of the project is the actually new question: given the benchmark results across the molecule set, can cheap chemical descriptors predict where the quantum-versus-surrogate gap opens, and which descriptors drive it? That question has a known qualitative answer but no systematic predictive model, and it is what BerryDelight is actually about.

The pipeline is Python-first, leaning on libraries like PennyLane and PySCF, with Rust reserved for the heavy lifting, beginning with a from-scratch selected-CI engine that serves as both a learning project and an independent cross-check.
