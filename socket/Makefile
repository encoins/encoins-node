all:
	docker compose up

stop:
	docker compose stop

scratch:
	docker compose build && docker compose up

listen:
	tshark -i any -Y "tcp.port in {12300..12400}"

clean:
	rm amialone.txt
