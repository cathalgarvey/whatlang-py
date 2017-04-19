from setuptools import setup, find_packages
from setuptools_rust import RustExtension

setup(name='whatlang',
    version='0.0.1',
    description="Simple python bindings to Sergey Potapov's whatlang-rs",
    long_description="This just wraps github.com/greyblake/whatlang-rs for Python 3.",
    author="Cathal Garvey",
    author_email="cathalgarvey@cathalgarvey.me",
    url='https://github.com/cathalgarvey/whatlang-py',
    license='MIT',
    packages=["whatlang"],
    rust_extensions=[RustExtension('whatlang.whatlang', 'whatlang-pybindings/Cargo.toml')],
    zip_safe=False,
    keywords='language recognition heuristic nlp',
    classifiers=[
        'Development Status :: 4 - Beta',
        'Intended Audience :: Developers',
        'Programming Language :: Rust',
        'Topic :: Text Processing :: Linguistic',
        'License :: OSI Approved :: MIT License',
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.4',
        'Programming Language :: Python :: 3.5',
        'Programming Language :: Python :: 3.6'
    ]
)
