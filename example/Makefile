a: b h_q
	@b_val=$$(head -n 1 b); \
	 h_q_val=$$(head -n 1 h_q); \
	 sum=$$((b_val + h_q_val)); \
	 echo $$sum > a

h_d: a
	@h_d_val=$$(head -n 1 a); \
	 echo $$h_d_val > h_d; \
	 od -An -N4 -tu4 /dev/urandom | tr -d ' ' > h_tag

h_q: h_tag
	@h_q_val=$$(head -n 1 h_d); \
	 echo $$h_q_val > h_q

clean:
	echo 0 > h_q > h_d > a
