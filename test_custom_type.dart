import 'dart:io';

void main() {
  try {
    print('Looking for generated Dart bindings...');
    final dir = Directory('target/debug/build');
    print('Contents of ${dir.path}:');
    dir.listSync(recursive: true).forEach((entity) {
      if (entity.path.endsWith('.dart')) {
        print('Found Dart file: ${entity.path}');
      }
    });
    
    print('\nTrying to import the bindings...');
    final dartFile = File('target/debug/build/uniffi_dart_test_lib-4a0124c54bb5a90f/out/uniffi_uniffi_dart_test_lib.dart');
    if (dartFile.existsSync()) {
      print('File exists, printing first few lines:');
      final content = dartFile.readAsStringSync().split('\n');
      for (int i = 0; i < 10 && i < content.length; i++) {
        print(content[i]);
      }
    } else {
      print('File does not exist');
    }
  } catch (e) {
    print('Error: $e');
  }
}
