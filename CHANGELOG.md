
<a name="v0.6.1"></a>

## [v0.6.1](https://github.com/MuntasirSZN/getquotes/compare/v0.6.0...v0.6.1) (2025-06-23)

### 🐞 Bug Fixes

- **security:** everything
  
  
<a name="v0.6.0"></a>

## [v0.6.0](https://github.com/MuntasirSZN/getquotes/compare/v0.5.1...v0.6.0) (2025-05-28)

### 🐞 Bug Fixes

- **cargo:** update dependencies to fix security vulnerabilities and use jiff instead of chrono
  
  
<a name="v0.5.1"></a>

## [v0.5.1](https://github.com/MuntasirSZN/getquotes/compare/v0.5.0...v0.5.1) (2025-04-27)

### 🐞 Bug Fixes

- use proper naming
  - use proper naming
  
  
<a name="v0.5.0"></a>

## [v0.5.0](https://github.com/MuntasirSZN/getquotes/compare/v0.4.0...v0.5.0) (2025-04-27)

### ✨ Features

- loads of tests
  - configuration file can now be specified via --config or -C
  
  ### 🐞 Bug Fixes

- tests
  - hombrew ci final
  - homebrew ci part 1
  - **tests:** properly handle Option<PathBuf>
  - **tests:** use home_dir() from std instead of HOME env var
  - **tests:** windows builds The reason is simple. The dirs crate uses windows apis to get the path, thus overriding it with temp_dir is impossible. std::env::home_dir was deprecated for a long long time. It was fixed in rust-lang/rust[#132515](https://github.com/MuntasirSZN/getquotes/issues/132515), but was still deprecated, thus showing warnings in edition 2024. In rust-lang/rust[#137327](https://github.com/MuntasirSZN/getquotes/issues/137327), it was undeprecated thus warnings won't show. Keep using the standard lib in this case. When next edition of rust releases, this will be solved.
  - **tests:** format code and make it flexible to tests
  
  
<a name="v0.4.0"></a>

## [v0.4.0](https://github.com/MuntasirSZN/getquotes/compare/v0.3.7...v0.4.0) (2025-04-13)

### ✨ Features

- completions, shorthands
  - brew ci script
  
  ### 🐞 Bug Fixes

- **ci:** windows build
  - **ci:** archive part
  - **ci:** use actions bot
  
  
<a name="v0.3.7"></a>

## [v0.3.7](https://github.com/MuntasirSZN/getquotes/compare/v0.3.6...v0.3.7) (2025-02-26)

### 🐞 Bug Fixes

- env
  
  
<a name="v0.3.6"></a>

## [v0.3.6](https://github.com/MuntasirSZN/getquotes/compare/v0.3.5...v0.3.6) (2025-02-26)

### ✨ Features

- homebrew and somenotes
  - **aur:** conflicts field, bump to 0.3.5, aarch64 support
  - **ci:** brew updater
  - **docs:** add TODO.md and .bacon-locations(bacon lsp)
  
  ### 🐞 Bug Fixes

- **ci:** environment vars
  
  
<a name="v0.3.5"></a>

## [v0.3.5](https://github.com/MuntasirSZN/getquotes/compare/v0.3.4...v0.3.5) (2025-02-19)

### ✨ Features

- **package:** Ready For Homebrew
  
  ### 🐞 Bug Fixes

- **lockfile:** cargo
  - **workflows:** dependency loop
  - **workflows:** needs test
  - **workflows:** tiny fix
  - **workflows:** final 3 step fix
  
  
<a name="v0.3.4"></a>

## [v0.3.4](https://github.com/MuntasirSZN/getquotes/compare/v0.3.3...v0.3.4) (2025-02-03)


<a name="v0.3.3"></a>

## [v0.3.3](https://github.com/MuntasirSZN/getquotes/compare/v0.3.2...v0.3.3) (2025-02-02)


<a name="v0.3.2"></a>

## [v0.3.2](https://github.com/MuntasirSZN/getquotes/compare/v0.3.1...v0.3.2) (2025-02-02)

### ✨ Features

- **workflow:** some verbosity
  - **workflow:** done making granular
  
  
<a name="v0.3.1"></a>

## [v0.3.1](https://github.com/MuntasirSZN/getquotes/compare/v0.3.0...v0.3.1) (2025-02-02)

### 🐞 Bug Fixes

- **workflow:** test code first
  - **workflow:** test code first
  
  
<a name="v0.3.0"></a>

## [v0.3.0](https://github.com/MuntasirSZN/getquotes/compare/v0.2.8...v0.3.0) (2025-02-02)

### 🐞 Bug Fixes

- **caching:** now cache is used perfectly
  - **rng:** depreacated funcs
  - **workflow:** test code first
  
  
<a name="v0.2.8"></a>

## [v0.2.8](https://github.com/MuntasirSZN/getquotes/compare/v0.2.7...v0.2.8) (2025-01-20)

### 🐞 Bug Fixes

- **workflow:** again
  
  
<a name="v0.2.7"></a>

## [v0.2.7](https://github.com/MuntasirSZN/getquotes/compare/v0.2.6...v0.2.7) (2025-01-20)

### 🐞 Bug Fixes

- **script:** handle pkgver only fix(workflow): use updpkgsums chore(release): update to 0.2.7
  
  
<a name="v0.2.6"></a>

## [v0.2.6](https://github.com/MuntasirSZN/getquotes/compare/v0.2.5...v0.2.6) (2025-01-18)


<a name="v0.2.5"></a>

## [v0.2.5](https://github.com/MuntasirSZN/getquotes/compare/v0.2.4...v0.2.5) (2025-01-18)

### ✨ Features

- better changelog
  - All packages are in packages dir feat: CI scripts are in ci dir feat: Experimental homebrew (working currently) fix(workflow): Comply with changes
  
  ### 🐞 Bug Fixes

- **workflow:** try
  
  
<a name="v0.2.4"></a>

## [v0.2.4](https://github.com/MuntasirSZN/getquotes/compare/v0.2.3...v0.2.4) (2025-01-18)

### 🐞 Bug Fixes

- **git-rev:** Dont panic!(), just return empty string
  - **readme:** Add versions, and some notes
  
  
<a name="v0.2.3"></a>

## [v0.2.3](https://github.com/MuntasirSZN/getquotes/compare/v0.2.2...v0.2.3) (2025-01-17)

### ✨ Features

- use actions better
  
  ### 🐞 Bug Fixes

- **ci:** script for pkg
  - **ci:** script for pkg
  
  
<a name="v0.2.2"></a>

## [v0.2.2](https://github.com/MuntasirSZN/getquotes/compare/v0.2.1...v0.2.2) (2025-01-17)

### 🐞 Bug Fixes

- pkgbuild ci
  - pkgbuild ci
  
  
<a name="v0.2.1"></a>

## [v0.2.1](https://github.com/MuntasirSZN/getquotes/compare/v0.2.0...v0.2.1) (2025-01-17)

### ✨ Features

- fresh start
  
  
<a name="v0.2.0"></a>

## [v0.2.0](https://github.com/MuntasirSZN/getquotes/compare/v0.1.2...v0.2.0) (2025-01-17)

### ✨ Features

- fresh start
  
  
<a name="v0.1.2"></a>

## [v0.1.2](https://github.com/MuntasirSZN/getquotes/compare/v0.1.1...v0.1.2) (2025-01-17)

### 🐞 Bug Fixes

- **workflow:** again
  - **workflow:** again
  
  
<a name="v0.1.1"></a>

## [v0.1.1](https://github.com/MuntasirSZN/getquotes/compare/v0.1.0...v0.1.1) (2025-01-17)

### ✨ Features

- Add AUR
  
  ### 🐞 Bug Fixes

- my name in cargo.toml
  - sqlite
  - everything
  - everything
  - **logger:** Now works fix(cache): add default path and fix feat: --version param
  - **tests:** Final fix
  - **tests:** Checking
  - **tests:** trying
  - **workflow:** Now better feat: PKGBUILD
  - **workflow:** Now better feat: PKGBUILD
  
  
<a name="v0.1.0"></a>

## v0.1.0 (2025-01-15)

### ✨ Features

- ci
  - cargo.toml
  - Readme, and others
  - Massive Changes
  
  ### 🐞 Bug Fixes

- everything
  - everything
  - everything
  - everything
  - android
  - android
  - reqwest
  - License
  
  