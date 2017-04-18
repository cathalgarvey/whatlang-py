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
      zip_safe=False
)
