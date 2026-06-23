# This is free and unencumbered software released into the public domain.

module Known; end
module Known::Countries; end

##
# A country (based on ISO 3166-1).
class Known::Countries::Country
  ##
  # The country code (ISO 3166-1).
  #
  # @return [Symbol]
  attr_reader :code

  private_class_method :new

  ##
  # Defines a new country.
  #
  # @param code [#to_sym] The country code (ISO 3166-1).
  def initialize(code)
    @code = code.to_sym
    self.freeze
  end

  ##
  # United Arab Emirates ("ae" in ISO 3166-1)
  #
  # @return [Country]
  UNITED_ARAB_EMIRATES = new(:ae)

  ##
  # Argentina ("ar" in ISO 3166-1)
  #
  # @return [Country]
  ARGENTINA = new(:ar)

  ##
  # Australia ("au" in ISO 3166-1)
  #
  # @return [Country]
  AUSTRALIA = new(:au)

  ##
  # Brazil ("br" in ISO 3166-1)
  #
  # @return [Country]
  BRAZIL = new(:br)

  ##
  # Canada ("ca" in ISO 3166-1)
  #
  # @return [Country]
  CANADA = new(:ca)

  ##
  # Switzerland ("ch" in ISO 3166-1)
  #
  # @return [Country]
  SWITZERLAND = new(:ch)

  ##
  # China ("cn" in ISO 3166-1)
  #
  # @return [Country]
  CHINA = new(:cn)

  ##
  # Germany ("de" in ISO 3166-1)
  #
  # @return [Country]
  GERMANY = new(:de)

  ##
  # Egypt ("eg" in ISO 3166-1)
  #
  # @return [Country]
  EGYPT = new(:eg)

  ##
  # Spain ("es" in ISO 3166-1)
  #
  # @return [Country]
  SPAIN = new(:es)

  ##
  # Finland ("fi" in ISO 3166-1)
  #
  # @return [Country]
  FINLAND = new(:fi)

  ##
  # France ("fr" in ISO 3166-1)
  #
  # @return [Country]
  FRANCE = new(:fr)

  ##
  # United Kingdom ("gb" in ISO 3166-1)
  #
  # @return [Country]
  UNITED_KINGDOM = new(:gb)

  ##
  # Greece ("gr" in ISO 3166-1)
  #
  # @return [Country]
  GREECE = new(:gr)

  ##
  # India ("in" in ISO 3166-1)
  #
  # @return [Country]
  INDIA = new(:in)

  ##
  # Italy ("it" in ISO 3166-1)
  #
  # @return [Country]
  ITALY = new(:it)

  ##
  # Japan ("jp" in ISO 3166-1)
  #
  # @return [Country]
  JAPAN = new(:jp)

  ##
  # South Korea ("kr" in ISO 3166-1)
  #
  # @return [Country]
  SOUTH_KOREA = new(:kr)

  ##
  # Mexico ("mx" in ISO 3166-1)
  #
  # @return [Country]
  MEXICO = new(:mx)

  ##
  # Netherlands ("nl" in ISO 3166-1)
  #
  # @return [Country]
  NETHERLANDS = new(:nl)

  ##
  # Norway ("no" in ISO 3166-1)
  #
  # @return [Country]
  NORWAY = new(:no)

  ##
  # New Zealand ("nz" in ISO 3166-1)
  #
  # @return [Country]
  NEW_ZEALAND = new(:nz)

  ##
  # Poland ("pl" in ISO 3166-1)
  #
  # @return [Country]
  POLAND = new(:pl)

  ##
  # Saudi Arabia ("sa" in ISO 3166-1)
  #
  # @return [Country]
  SAUDI_ARABIA = new(:sa)

  ##
  # Sweden ("se" in ISO 3166-1)
  #
  # @return [Country]
  SWEDEN = new(:se)

  ##
  # Singapore ("sg" in ISO 3166-1)
  #
  # @return [Country]
  SINGAPORE = new(:sg)

  ##
  # Turkey ("tr" in ISO 3166-1)
  #
  # @return [Country]
  TURKEY = new(:tr)

  ##
  # Ukraine ("ua" in ISO 3166-1)
  #
  # @return [Country]
  UKRAINE = new(:ua)

  ##
  # United States ("us" in ISO 3166-1)
  #
  # @return [Country]
  UNITED_STATES = new(:us)

  ##
  # South Africa ("za" in ISO 3166-1)
  #
  # @return [Country]
  SOUTH_AFRICA = new(:za)
end # Known::Countries::Country
