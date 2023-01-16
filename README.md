## What is Formula-Inter

A very simple Interpreter made in Rust that understand this grammar

SCRIPT → LISTINSTR

LISTINSTR → INSTR LISTINSTR

LISTINSTR → ε

INSTR → id = PD_AFF ;

PD_AFF → E

PD_AFF → inv E

PD_AFF → sqrt E

INSTR → loop E { LISTINSTR }

INSTR → afficher E ;

INSTR → aff_ral ; (commentaire : affiche un retour à la ligne)

E → T D

D → + E

D → ε

T → F G

G → * T

G → ε

F → ( E )

F → nb

F → id

## How To Use

Clone this repository:

```bash
git clone https://github.com/hectellian/formula-inter
```

Compile the binary file


```bash
cargo build --release
```
Then, in the command line juste type:

```bash
./target/release/formula-inter <file.fi>
```

---

Or (if you already have the binary file)

```bash
./formula-inter <file.fi>
```
