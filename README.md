Seax Virtual Machine (SVM)
==========================

[![Join the chat at https://gitter.im/hawkw/seax](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/hawkw/seax?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

[![Build Status](https://img.shields.io/travis/hawkw/seax_svm/master.svg?style=flat-square)](https://travis-ci.org/hawkw/seax_svm)
[![Coverage](https://img.shields.io/codecov/c/github/hawkw/seax_svm/master.svg?style=flat-square)](http://codecov.io/github/hawkw/seax_svm?branch=master)
[![Latest RustDoc](https://img.shields.io/badge/rustdoc-latest-green.svg?style=flat-square)](http://hawkweisman.me/seax/api/seax_svm/)
[![Latest SVM release](https://img.shields.io/crates/v/seax_svm.svg?style=flat-square)](https://crates.io/crates/seax_svm)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](https://github.com/hawkw/seax/LICENSE)

The [Seax](http://hawkweisman.me/seax/) virtual machine, a runtime environment for functional programming languages.

Seax is a virtual machine based on the [SECD machine](http://en.wikipedia.org/wiki/SECD_machine) described by Peter Landin in 1963. It is intended as a portible, robust run-time environment to be used as an implementation target for programming languages, with an emphasis on functional programming.

SVM is distributed as a library so that a Seax runtime may be embedded in other software systems. That means that the code in this repository is not sufficient to compile and run Scheme programs for Seax on its own. The [hawkw/seax](https://github.com/hawkw/seax) repository contains the Seax command-line application, which you will probably want if you intend to develop programs targeting Seax. Currently, a [Scheme interpreter/compiler]((https://github.com/hawkw/seax_scheme)) is under active development, with other programming languages targeting Seax coming eventually.

Please report all issues and feature requests to the main repository ([hawkw/seax](https://github.com/hawkw/seax)).

Contributing
------------

Seax is an open-source project and contributions are happily welcomed. For more information on how to contribute to Seax, please see the [CONTRIBUTING](https://github.com/hawkw/seax/blob/master/CONTRIBUTING.md) document on the main Seax repository.
