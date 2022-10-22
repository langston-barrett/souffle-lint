import lit.formats

config.name = "souffle-lint"
config.test_format = lit.formats.ShTest(True)
config.suffixes = [".dl", ".yml"]
