// This is free and unencumbered software released into the public domain.

/// A country (based on ISO 3166-1).
enum Country {

  /// United Arab Emirates ("ae" in ISO 3166-1)
  unitedArabEmirates("ae"),

  /// Argentina ("ar" in ISO 3166-1)
  argentina("ar"),

  /// Australia ("au" in ISO 3166-1)
  australia("au"),

  /// Brazil ("br" in ISO 3166-1)
  brazil("br"),

  /// Canada ("ca" in ISO 3166-1)
  canada("ca"),

  /// Switzerland ("ch" in ISO 3166-1)
  switzerland("ch"),

  /// China ("cn" in ISO 3166-1)
  china("cn"),

  /// Germany ("de" in ISO 3166-1)
  germany("de"),

  /// Egypt ("eg" in ISO 3166-1)
  egypt("eg"),

  /// Spain ("es" in ISO 3166-1)
  spain("es"),

  /// Finland ("fi" in ISO 3166-1)
  finland("fi"),

  /// France ("fr" in ISO 3166-1)
  france("fr"),

  /// United Kingdom ("gb" in ISO 3166-1)
  unitedKingdom("gb"),

  /// Greece ("gr" in ISO 3166-1)
  greece("gr"),

  /// India ("in" in ISO 3166-1)
  india("in"),

  /// Italy ("it" in ISO 3166-1)
  italy("it"),

  /// Japan ("jp" in ISO 3166-1)
  japan("jp"),

  /// South Korea ("kr" in ISO 3166-1)
  southKorea("kr"),

  /// Mexico ("mx" in ISO 3166-1)
  mexico("mx"),

  /// Netherlands ("nl" in ISO 3166-1)
  netherlands("nl"),

  /// Norway ("no" in ISO 3166-1)
  norway("no"),

  /// New Zealand ("nz" in ISO 3166-1)
  newZealand("nz"),

  /// Poland ("pl" in ISO 3166-1)
  poland("pl"),

  /// Saudi Arabia ("sa" in ISO 3166-1)
  saudiArabia("sa"),

  /// Sweden ("se" in ISO 3166-1)
  sweden("se"),

  /// Singapore ("sg" in ISO 3166-1)
  singapore("sg"),

  /// Turkey ("tr" in ISO 3166-1)
  turkey("tr"),

  /// Ukraine ("ua" in ISO 3166-1)
  ukraine("ua"),

  /// United States ("us" in ISO 3166-1)
  unitedStates("us"),

  /// South Africa ("za" in ISO 3166-1)
  southAfrica("za"),;

  /// The country code (ISO 3166-1).
  final String code;

  /// Defines a new country.
  const Country(this.code);

  /// Creates a [Country] from the given ISO 3166-1 code.
  factory Country.fromCode(String code) {
    return Country.values.firstWhere(
      (lang) => lang.code == code,
      orElse: () => throw ArgumentError('Unknown ISO 3166-1 code: $code'),
    );
  }
}
