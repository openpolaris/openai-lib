generate:
	openapi-generator generate -g rust --skip-validate-spec \
		-i https://raw.githubusercontent.com/openai/openai-openapi/master/openapi.yaml \
		--package-name openai-lib --additional-properties=packageVersion=0.1.0
