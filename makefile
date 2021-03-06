IMAGE_NAME = msc_api

build: update image

update :
	git pull

build-database :
	bash ./build_database.sh

build-app : 
	cargo build --release

build-image : build-app build-database
	sudo docker image rm $(IMAGE_NAME) || true && \
		sudo docker build -t $(IMAGE_NAME) .

image : build-image
	sudo docker save $(IMAGE_NAME) --output $(IMAGE_NAME).tar && \
		sudo chown mick:mick $(IMAGE_NAME).tar && \
		zip $(IMAGE_NAME).tar.zip $(IMAGE_NAME).tar && \
		rm $(IMAGE_NAME).tar

clean : 
	rm $(IMAGE_NAME).tar $(IMAGE_NAME).tar.zip||true && cargo clean
