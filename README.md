<div id="top"></div>

[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/UK-IPOP/drug-extraction">
    <img src="images/logo.png" alt="Logo">
  </a>

  <h3 align="center">Drug Extraction ToolBox</h3>

  <p align="center">
    A suite of tools to extract drugs from text records.
    <br />
    <br />
    <a href="https://github.com/uk-ipop/drug-extraction"><strong>Explore the docs >></strong></a>
    <br />
    <a href="https://github.com/UK-IPOP/drug-extraction/issues/new">Report Bug</a>
    ·
    <a href="https://github.com/UK-IPOP/drug-extraction/issues/new">Request Feature</a>
    <br />
    <a href="https://github.com/UK-IPOP/drug-extraction/tree/main/cli">CLI</a>
    ·
    <a href="https://github.com/UK-IPOP/drug-extraction/tree/main/core">Core</a>
    ·
    <a href="https://github.com/UK-IPOP/drug-extraction/tree/main/web">Web</a>
  </p>
</div>

<div align="center">

[![WEBSITE][play-shield]][play-url]

</div>

This project uses string similarity metrics to detect drugs inside text records.

Researchers at the University of Kentucky had a need for a simple, fast, intuitive interface to extract drug mentions in text records. This project takes text records and detects drug mentions (including misspellings) and then extracts the drug and the corresponding record for analysis.

<p align="right">(<a href="#top">back to top</a>)</p>

### Built With

- [Rust](https://www.rust-lang.org) 🦀
- [wasm-pack](https://github.com/rustwasm/wasm-pack) 🕸
- [Next.js](https://nextjs.org/)
- [Bootstrap](https://getbootstrap.com)

<p align="right">(<a href="#top">back to top</a>)</p>

## Getting Started

In order to set clear expectations for the development of the toolbox and to keep feature requests in scope it is good to set some guidelines.

What this project IS:

- A suite of tools to extract drug mentions from text records
- A string parsing tool
- A website

What this project is NOT:

- An analytical tool
- A business intelligence tool
- A preprocessing tool

We've chosen to utilize a mono-repo format for this project. This is our first implementation of the mono-repo structure when using Rust. If it causes more problems than benefits, we will switch to independent repositories for each project.

This toolbox contains multiple projects:

- A command line tool (available via cargo install)
- A core Rust library for parsing strings and comparing them to common drugs
  - This is also configurable for custom search options and integration with the popular [RxNorm](https://www.nlm.nih.gov/research/umls/rxnorm/index.html) library from the National Library of Medicine ([NLMS](https://www.nlm.nih.gov))
  - This also contains web assembly bindings
- A website utilizing the core Web Assembly bindings

> **The command-line application will always be given priority bug and feature support as it is the most versatile. The website will always come second as it is simply a thin interface-wrapper around the CLI/Core libraries for non-technical researchers. Core library development will be driven _strongly_ by the needs of the CLI and thus may introduce breaking changes so please be sure to pin your version.**
>
> For more information, consult the corresponding tool's documentation [CLI](cli/README.md) -- [Core](core/README.md) -- [Web](web/README.md). 😃

We utilize string similarity algorithms as defined and implemented by [Danny Guo](https://github.com/dguo) in the [str-sim](https://github.com/dguo/strsim-rs) package. For more information on string similarity algorithms, please consult [this](https://en.wikipedia.org/wiki/String_metric) Wikipedia page for a **comparison/list** of algorithms and [this](https://en.wikipedia.org/wiki/Edit_distance) page for an explanation of string metrics more generally.

<p align="right">(<a href="#top">back to top</a>)</p>

## Roadmap

- [ ] Add Changelog
- [ ] Add tests
- [ ] Add CI/CD
- [ ] Add Additional Examples
- [ ] For more, see [Issues](https://github.com/uk-ipop/drug-extraction/issues)

See the [open issues](https://github.com/othneildrew/Best-README-Template/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#top">back to top</a>)</p>

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

We use `gh release create` to make new github releases and `cargo release` to release to crates.io.

<p align="right">(<a href="#top">back to top</a>)</p>

## License

Distributed under the MIT License. See `LICENSE` for more information.

<p align="right">(<a href="#top">back to top</a>)</p>

## Contact

Nick Anthony - [@nanthony95](https://twitter.com/nanthony95) - nicholas.anthony@uky.edu

Project Link: [https://github.com/uk-ipop/drug-extraction](https://github.com/uk-ipop/drug-extraction)

<p align="right">(<a href="#top">back to top</a>)</p>

## Acknowledgments

- [Choose an Open Source License](https://choosealicense.com)
- [Img Shields](https://shields.io)
- [GitHub Pages](https://pages.github.com)
- [RustBook](https://doc.rust-lang.org/book/)

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[stars-shield]: https://img.shields.io/github/stars/uk-ipop/drug-extraction?style=for-the-badge
[stars-url]: https://github.com/uk-ipop/drug-extraction/stargazers
[issues-shield]: https://img.shields.io/github/issues/uk-ipop/drug-extraction?style=for-the-badge
[issues-url]: https://github.com/uk-ipop/drug-extraction/issues
[license-shield]: https://img.shields.io/github/license/uk-ipop/drug-extraction.svg?style=for-the-badge
[license-url]: https://github.com/uk-ipop/drug-extraction/blob/master/LICENSE.txt
[play-shield]: https://img.shields.io/badge/Website-blue?style=for-the-badge

<!-- TODO: switch to Github Pages / Vercel when published -->

[play-url]: https://github.com/UK-IPOP/drug-extraction
