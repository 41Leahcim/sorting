app: app.c
	gcc app.c -Wall -Wextra -O2 -o app

run: app
	./app

clean:
	rm app

analyze:
	gcc app.c -Wall -Wextra -O2 -fanalyzer -o app
