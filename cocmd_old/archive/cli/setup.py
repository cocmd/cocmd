#!/usr/bin/env python
# -*- coding: utf-8 -*-
from setuptools import setup, find_packages

with open('README.md') as readme_file:
    readme = readme_file.read()

requirements = [
    'Click>=6.0',
    'rich',
    'inquirer',
    'dacite',
    'pyyaml'
]

setup(
    name='cocmd1',
    version='0.1.19',
    description="CLI tool boilerplate using click, please replace!",
    long_description='bla bla',
    author="Moshe Ro",
    author_email='mzsrtgzr2@gmail.com',
    url='https://github.com/cocmd/cocmd',
    packages=['cocmd_cli_app'],
    entry_points={
        'console_scripts': [
            'cocmd=cocmd_cli_app:cli'
        ]
    },
    include_package_data=True,
    install_requires=requirements,
    zip_safe=False,
    keywords='cocmd1',
    classifiers=[
    ]
)
