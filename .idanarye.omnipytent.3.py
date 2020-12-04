import vim
from omnipytent import *
from omnipytent.ext.idan import *

import re


@task
def compile(ctx):
    cargo['build', '-q'] & ERUN.bang


@task
def run(ctx):
    # cargo['aoc'] & TERMINAL_PANEL.size(15)
    cargo['run'] & BANG


@task
def download_input(ctx):
    cargo['aoc', 'input'] & BANG


@task(alias=':0')
def download_missing_input(ctx):

    def gen(path, pattern):
        pattern = re.compile(pattern)
        for p in local.path(path):
            if m:= pattern.search(p.basename):
                day_text, = m.groups()
                yield day_text

    inputs = set(gen('input/2020', r'day(\d+).txt'))
    days = set(gen('src', r'day(\d+).rs'))
    for day in days - inputs:
        cargo['aoc', 'input', '-d', day] & SH
