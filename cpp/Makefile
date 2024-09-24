CXX=clang++

all: build

main: main.cpp
	$(CXX) -o $@ -std=c++17 -Wall -Wpedantic -O2 $^

build: main

run: main
	./$^

debug: main.cpp
	g++ -o main -Wall -Wpedantic -ggdb2 $^

clean:
	rm main
