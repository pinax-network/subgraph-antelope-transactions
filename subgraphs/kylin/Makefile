.PHONY: all
all:
	make build

.PHONY: build
build:
	graph build

.PHONY: deploy
deploy:
	graph create --node http://localhost:8020 kylin_transactions
	graph deploy --node=http://localhost:8020 kylin_transactions

.PHONY: publish
publish:
	graph publish --subgraph-id GpJpCvfozP3XWQRomagcU8t4tzmU5A788VNqKZuR2rCb

.PHONY: gui
gui:
	substreams gui ../../substreams.yaml -e kylin.substreams.pinax.network:443 graph_out -s 373006623 -t 0 --production-mode