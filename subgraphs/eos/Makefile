.PHONY: all
all:
	make build

.PHONY: build
build:
	graph build

.PHONY: deploy
deploy:
	graph create --node http://localhost:8020 eos_transactions
	graph deploy --node=http://localhost:8020 eos_transactions

.PHONY: publish
publish:
	graph publish --subgraph-id 2RNdhL5p62dGN5UqKtsSEhYZiTJbFcuuhzk9qRJj8QeU

.PHONY: gui
gui:
	substreams gui ../../substreams.yaml -e eos.substreams.pinax.network:443 graph_out -s 396143482 -t 0 --production-mode