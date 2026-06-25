abort("Expected Ruby 3.4+, but got #{RUBY_VERSION}.") if RUBY_VERSION < '3.4.0'

require 'csv'
require 'lvr'

Country = Data.define(:alpha2, :alpha3, :name) do
  def to_liquid = self.to_h.merge(code: alpha2).transform_keys(&:to_s)
end

codegen = ->(t, _) { Lvr.codegen(t.name, t.source, **CONTEXT) }
copy = ->(t, _) { cp t.source, t.name }

task :default => %w[README.md dart python ruby rust]
file 'README.md' => %w[.config/codegen/README.md.liquid], &codegen

task dart: %w[dart/README.md dart/lib/src/country.dart]
file 'dart/README.md' => %w[.config/codegen/dart/README.md.liquid data/countries.csv], &codegen
file 'dart/lib/src/country.dart' => %w[.config/codegen/dart/country.liquid data/countries.csv], &codegen # TODO: `dart format`

task python: %w[python/README.md python/src/known_countries/__init__.py]
file 'python/README.md' => %w[.config/codegen/python/README.md.liquid data/countries.csv], &codegen
file 'python/src/known_countries/__init__.py' => %w[.config/codegen/python/country.liquid data/countries.csv], &codegen

task ruby: %w[ruby/README.md ruby/lib/known/countries.rb]
file 'ruby/README.md' => %w[.config/codegen/ruby/README.md.liquid data/countries.csv], &codegen
file 'ruby/lib/known/countries.rb' => %w[.config/codegen/ruby/country.liquid data/countries.csv], &codegen
file 'ruby/CHANGES.md' => %w[CHANGES.md], &copy
file 'ruby/VERSION' => %w[VERSION], &copy

task rust: %w[rust/README.md rust/src/country.rs]
file 'rust/README.md' => %w[.config/codegen/rust/README.md.liquid data/countries.csv], &codegen
file 'rust/src/country.rs' => %w[.config/codegen/rust/country.liquid data/countries.csv], &codegen

def load_countries = parse_csv('data/countries.csv').map { Country.new(*it) }
def parse_csv(path) = CSV.parse(File.read(path), headers: false)

CONTEXT = {
  project: {
    title: "Known Countries",
  },
  github: {
    repository: {
      link: 'https://github.com/it-is-known/known-countries',
      url:  'https://github.com/it-is-known/known-countries.git',
    }
  },
  countries: load_countries,
}
