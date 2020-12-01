import vim
from omnipytent import *
from omnipytent.ext.idan import *


@task
def compile(ctx):
    cargo['build', '-q'] & ERUN.bang


@task
def run(ctx):
    cargo['run', '-q'] & BANG


@task
def download_input(ctx):
    cargo['aoc', 'input'] & BANG
