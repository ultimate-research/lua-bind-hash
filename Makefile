all: rust c

c: lua_bind_hash.c
	@gcc lua_bind_hash.c -o lua_bind_hash_c

rust: lua_bind_hash.rs
	@rustc -C debug-assertions=off lua_bind_hash.rs -o lua_bind_hash_rs

clean:
	rm lua_bind_hash_rs lua_bind_hash_c
