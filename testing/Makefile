try: main.c
	clang -o $@ $^ -I/home/brent/omsf/clone/openmm/build/wrappers -l OpenMM -L/home/brent/omsf/clone/openmm/build

run: try
	LD_LIBRARY_PATH=/home/brent/omsf/clone/openmm/build ./try
