#!/usr/bin/env python3

import os
import math
import json
import sys
import argparse
from pathlib import Path
from collections import defaultdict

RUST_OPERATORS = {
    'fn', 'let', 'if', 'else', 'match', '=', '==', '!=', '<', '>', '<=', '>=',
    '+', '-', '*', '/', '%', '&', '|', '!', '&&', '||', '->', '=>', ';', ',',
    '.', ':', '::', '{', '}', '(', ')', '[', ']', '<<', '>>', '?', 'pub', 'mod',
    'use', 'struct', 'enum', 'impl', 'trait', 'where', 'for', 'in', 'while',
    'loop', 'return', 'break', 'continue', 'async', 'await', 'move', 'ref',
    'mut', 'const', 'static', 'unsafe', 'extern', 'crate', 'self', 'super',
    'type', 'dyn', 'box', 'as', 'true', 'false', 'None', 'Some', 'Ok', 'Err'
}

def is_operator(token):
    return token in RUST_OPERATORS

def is_operand(token):
    return (token and 
            not is_operator(token) and
            not token.startswith('"') and
            not token.startswith("'") and
            not token.startswith('//') and
            all(c.isalnum() or c == '_' for c in token))

def calculate_halstead_metrics(code):
    operators = set()
    operands = set()
    operator_count = 0
    operand_count = 0
    
    lines = code.split('\n')
    for line in lines:
        line = line.strip()
        if line.startswith('//') or line.startswith('/*'):
            continue
            
        if '//' in line:
            line = line.split('//')[0]
            
        tokens = line.split()
        for token in tokens:
            if is_operator(token):
                operators.add(token)
                operator_count += 1
            elif is_operand(token):
                operands.add(token)
                operand_count += 1
    
    n1 = len(operators)
    n2 = len(operands)
    N1 = operator_count
    N2 = operand_count
    
    vocabulary = n1 + n2
    length = N1 + N2
    volume = length * math.log2(vocabulary) if vocabulary > 0 else 0
    difficulty = (n1 / 2) * (N2 / n2) if n2 > 0 else 0
    effort = difficulty * volume
    time = effort / 18
    bugs = volume / 3000
    
    return {
        'n1': n1,
        'n2': n2,
        'vocabulary': vocabulary,
        'length': length,
        'volume': round(volume, 2),
        'difficulty': round(difficulty, 2),
        'effort': round(effort, 2),
        'time': round(time, 2),
        'bugs': round(bugs, 4),
        'N1': N1,
        'N2': N2
    }

def analyze_workspace(workspace_path):
    total_metrics = defaultdict(int)
    total_metrics['volume'] = 0.0
    total_metrics['difficulty'] = 0.0
    total_metrics['effort'] = 0.0
    total_metrics['bugs'] = 0.0
    
    file_metrics = []
    file_count = 0
    
    for root, dirs, files in os.walk(workspace_path):
        if 'target' in dirs:
            dirs.remove('target')
        
        if 'tests' in dirs:
            dirs.remove('tests')
            
        for file in files:
            if file.endswith('.rs'):
                file_path = Path(root) / file
                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        code = f.read()
                    
                    metrics = calculate_halstead_metrics(code)
                    metrics['file'] = str(file_path)
                    file_metrics.append(metrics)
                    
                    total_metrics['n1'] += metrics['n1']
                    total_metrics['n2'] += metrics['n2']
                    total_metrics['N1'] += metrics['N1']
                    total_metrics['N2'] += metrics['N2']
                    total_metrics['volume'] += metrics['volume']
                    total_metrics['difficulty'] += metrics['difficulty']
                    total_metrics['effort'] += metrics['effort']
                    total_metrics['bugs'] += metrics['bugs']
                    file_count += 1
                    
                except Exception as e:
                    print(f"Error analyzing {file_path}: {e}")
    
    return file_metrics, dict(total_metrics), file_count

def print_metrics(metrics, title=""):
    if title:
        print(f"\n{title}")
    print(f"  Operators: {metrics['n1']} unique, {metrics['N1']} total")
    print(f"  Operands:  {metrics['n2']} unique, {metrics['N2']} total")
    print(f"  Vocabulary: {metrics['vocabulary']}")
    print(f"  Length: {metrics['length']}")
    print(f"  Volume: {metrics['volume']}")
    print(f"  Difficulty: {metrics['difficulty']}")
    print(f"  Effort: {metrics['effort']}")
    print(f"  Time: {metrics['time']} seconds")
    print(f"  Estimated bugs: {metrics['bugs']}")

AVAILABLE_AVG_DIFFICULTY=20
parser = argparse.ArgumentParser(description='Analyze Halstead complexity metrics for Rust workspace')
parser.add_argument('path', nargs='?', default='.', 
                    help='Path to Rust code directory (default: current directory)')
parser.add_argument('--threshold', '-t', type=float, default=AVAILABLE_AVG_DIFFICULTY,
                    help=f'Average difficulty threshold (default: {AVAILABLE_AVG_DIFFICULTY})')
parser.add_argument('--output', '-o', default='halstead_metrics.json',
                    help='Output JSON file (default: halstead_metrics.json)')

def main():
    args = parser.parse_args()
    
    workspace_path = args.path
    threshold = args.threshold
    output_file = args.output
    
    if not os.path.exists(workspace_path):
        print(f"Error: Path '{workspace_path}' does not exist")
        sys.exit(1)
    
    print(f"Analyzing Rust in directory: {workspace_path}")
    
    file_metrics, total_metrics, file_count = analyze_workspace(workspace_path)
    
    avg_difficulty = total_metrics['difficulty'] / file_count
    if file_count > 0:
        print(f"\n{'='*45}")
        print("СВОДКА")
        print(f"{'='*45}")
        print(f"Проанализировано файлов: {file_count}")
        print(f"Всего операторов: {total_metrics['N1']}")
        print(f"Всего операндов: {total_metrics['N2']}")
        print(f"Общий объем: {total_metrics['volume']:.2f}")
        print(f"Средний объем на файл: {total_metrics['volume'] / file_count:.2f}")
        print(f"Средняя сложность: {avg_difficulty:.2f}")
        print(f"Общие усилия: {total_metrics['effort']:.2f}")
        print(f"Ожидаемое количество ошибок: {total_metrics['bugs']:.4f}")

    exit_code = 0
    if avg_difficulty > threshold:
        exit_code = 1
    
    with open(output_file, 'w') as f:
        json.dump({
            'summary': {
                'files_analyzed': file_count,
                'total_volume': total_metrics['volume'],
                'total_effort': total_metrics['effort'],
                'estimated_bugs': total_metrics['bugs'],
                'total_difficulty': total_metrics['difficulty'],
                'avg_difficulty': avg_difficulty
            },
            'files': file_metrics
        }, f, indent=2)
    
    print(f"\nDetailed metrics saved to halstead_metrics.json")
    sys.exit(exit_code)

if __name__ == "__main__":
    main()