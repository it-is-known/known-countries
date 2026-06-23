require 'csv'
require 'liquid'
require 'tilt'

HEADER = File.open('UNLICENSE').readlines.first

Country = Data.define(:alpha2, :alpha3, :name) do
  def to_liquid
    { 'code' => alpha2, 'alpha2' => alpha2, 'alpha3' => alpha3, 'name' => name }
  end
end

module Liquid::StandardFilters
  def constcase(input)
    input.split(' ').map(&:upcase).join('_')
  end

  def camelcase(input)
    output = pascalcase(input)
    output[0].downcase + output[1..]
  end

  def pascalcase(input)
    input.split(' ').map(&:capitalize).join('')
  end
end # Liquid::StandardFilters

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

def codegen_readme(target)
  countries = load_countries()
  template = load_template(".config/codegen/#{target}/README.md.liquid")
  template.render!({ 'countries' => countries },
    { error_mode: :strict, strict_variables: true, strict_filters: true })
end

def codegen_countries(target)
  countries = load_countries()
  template = load_template(".config/codegen/#{target}/country.liquid")
  template.render!({ 'countries' => countries },
    { error_mode: :strict, strict_variables: true, strict_filters: true })
end

def load_template(path) = Liquid::Template.parse(File.read(path))

def load_countries() = parse_csv('data/countries.csv')
  .map { |(alpha2, alpha3, name)| Country.new(alpha2, alpha3, name) }

def parse_csv(path) = CSV.parse(File.read(path), headers: false)
