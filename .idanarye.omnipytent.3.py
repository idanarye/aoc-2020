import vim
from omnipytent import *
from omnipytent.ext.idan import *

import re


def gen_all_implemented_days():
    pattern = re.compile(r'day(\d+).rs')
    for p in local.path('src'):
        if m:= pattern.search(p.basename):
            day_text, = m.groups()
            yield int(day_text)


@task
def compile(ctx):
    cargo['build', '-q'] & ERUN.bang


@task
def run(ctx):
    cargo['run', '--', '--day', str(max(gen_all_implemented_days()))] & BANG


@task
def act(ctx):
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
    main_file = local.path('src/main.rs')

    def gen_main_lines():
        day_pattern = re.compile(r'^\s*day(\d+)\s*:.*;\s*$');
        it = iter(main_file.read('utf8').splitlines())

        for line in it:
            yield line
            if day_pattern.match(line):
                break
        else:
            assert False, 'found no end?'

        for line in it:
            if not day_pattern.match(line):
                break
            yield line
        else:
            assert False, 'found no end?'

        yield f'    day{day_nr} : generator => part_1, part_2;'
        yield line
        yield from it

    def gen_lib_lines():
        yield from lib_main_file.read('utf8').splitlines()
        yield f'pub mod day{day_nr};'

    main_file.write('\n'.join(gen_main_lines()))
    lib_main_file.write('\n'.join(gen_lib_lines()))

    CMD.checktime()
