flags = -Wall -Wextra -Wpedantic -Wtrampolines -fPIE -pie -fcf-protection=full

app: app.cpp
	g++ app.cpp $(flags) -O2 -o app

clean:
	rm app

analyze: app.cpp
	g++ app.cpp $(flags) -fanalyzer -O2 -o app
