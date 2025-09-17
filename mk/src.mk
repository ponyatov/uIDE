# .mk files
MK += Makefile $(wildcard mk/*.mk)
MK += $(wildcard   hw/*/*.mk)
MK += $(wildcard  cpu/*/*.mk)
MK += $(wildcard arch/*/*.mk)
MK += $(wildcard   os/*/*.mk)

# C/C++
C  += $(wildcard src/*.c*)
H  += $(wildcard inc/*.h*)

# Rust
R += Cargo.toml $(wildcard src/*.rs)

# ini
S  += $(wildcard lib/*.ini) $(wildcard lib/*.f)

