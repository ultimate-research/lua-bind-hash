all: rust c cs

c: lua_bind_hash.c
	@gcc lua_bind_hash.c -o lua_bind_hash_c

cs: lua_bind_hash.cs
	@mcs -unsafe lua_bind_hash.cs

rust: lua_bind_hash.rs
	@rustc -C debug-assertions=off lua_bind_hash.rs -o lua_bind_hash_rs

clean:
	rm -f lua_bind_hash_rs lua_bind_hash_c lua_bind_hash.exe

test: all
	@printf "\nC\n-----\n"
	@./lua_bind_hash_c
	@printf "\nRust\n-----\n"
	@./lua_bind_hash_rs
	@printf "\nC#\n-----\n"
	@mono lua_bind_hash.exe

