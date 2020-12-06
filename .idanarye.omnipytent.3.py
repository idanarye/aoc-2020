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


@task
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


@task
def add_day(ctx, day_nr):
    import copier
    copier.copy(
        '.copier/day-template',
        '.',
        dict(day=day_nr))

    lib_main_file = local.path('src/lib.rs')

    def gen_new_lines():
        day_mod_pattern = re.compile(r'^pub mod day\d+;$');
        it = iter(lib_main_file.read('utf8').splitlines())

        for line in it:
            if not day_mod_pattern.match(line):
                break
            yield line
        else:
            assert False, 'found no end?'

        yield f'pub mod day{day_nr};'
        yield line

        yield from it

    lib_main_file.write('\n'.join(gen_new_lines()))
