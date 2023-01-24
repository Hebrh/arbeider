"""Setup configuration for the package."""
# -*- coding: utf-8 -*-
from setuptools import find_packages, setup
import versioneer

# pylint: disable=invalid-name
long_desc = """
arbeider is a python function runtime framework,
which can be used to run python functions in a rust executor.
"""

setup(
    name="arbeider",
    version=versioneer.get_version(),
    cmdclass=versioneer.get_cmdclass(),
    url="https://github.com/lipicoder/arbeider",
    author="lipi,zhujw",
    author_email="lipicoder@qq.com, kratoswittgenstein@gmail.com",
    description="Exchange data between database and parquet files",
    long_description=long_desc,
    zip_safe=True,
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    packages=find_packages(),
    include_package_data=True,
)
