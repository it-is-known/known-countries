require 'csv'
require 'lvr'

Country = Data.define(:alpha2, :alpha3, :name) do
  def to_liquid = self.to_h.merge(code: alpha2).transform_keys(&:to_s)
end

task :default => %w[dart python ruby rust]

task dart: %w[dart/README.md dart/lib/src/country.dart]

file 'dart/README.md' => 'data/countries.csv' do |t|
  File.open(t.name, 'w') { it.puts codegen_readme(:dart) }
end

file 'dart/lib/src/country.dart' => 'data/countries.csv' do |t|
  File.open(t.name, 'w') { it.puts codegen_countries(:dart) } # TODO: `dart format`
end

task python: %w[python/README.md python/src/known_countries/__init__.py]

file 'python/README.md' => 'data/countries.csv' do |t|
  File.open(t.name, 'w') { it.puts codegen_readme(:python) }
end

file 'python/src/known_countries/__init__.py' => 'data/countries.csv' do |t|
  File.open(t.name, 'w') { it.puts codegen_countries(:python) }
end

task ruby: %w[ruby/README.md ruby/lib/known/countries.rb]

file 'ruby/README.md' => 'data/countries.csv' do |t|
  File.open(t.name, 'w') { it.puts codegen_readme(:ruby) }
end

file 'ruby/lib/known/countries.rb' => 'data/countries.csv' do |t|
  File.open(t.name, 'w') { it.puts codegen_countries(:ruby) }
end

task rust: %w[rust/README.md rust/src/country.rs]

file 'rust/README.md' => 'data/countries.csv' do |t|
  File.open(t.name, 'w') { it.puts codegen_readme(:rust) }
end

file 'rust/src/country.rs' => 'data/countries.csv' do |t|
  File.open(t.name, 'w') { it.puts codegen_countries(:rust) } # TODO: `rustfmt`
end

def codegen_readme(target) = Lvr::Template
  .load(".config/codegen/#{target}/README.md.liquid")
  .render(countries: load_countries())

def codegen_countries(target) = Lvr::Template
  .load(".config/codegen/#{target}/country.liquid")
  .render(countries: load_countries())

def load_countries() = parse_csv('data/countries.csv')
  .map { |(alpha2, alpha3, name)| Country.new(alpha2, alpha3, name) }

def parse_csv(path) = CSV.parse(File.read(path), headers: false)
