PROJECT_NAME	:= $(shell basename "$(PWD)")
TAG    			:= $(shell git log -1 --pretty=%H)
IMG    			:= ${PROJECT_NAME}:${TAG}

default: build

build:
	@echo "Build completed successfully"

clean:
	@echo "Cleanup successful!"
