
# PythonKyber

Привязки Python для постквантовой криптографии Kyber с использованием pqcrypto
## Пару слов про алгоритм Kyber

Kyber — это схема постквантового шифрования, основанная на решении задачи «среднего проблемы» в теории вычислительной сложности, известной как «проблема вычисления полиномиального представления». Он использует математическую структуру, называемую решётками, и является одной из рекомендованных схем для использования в постквантовых системах безопасности, поскольку он предполагает стойкость против атак с использованием квантовых компьютеров.

Вот как работает Kyber на высоком уровне:

1. **Ключи:**
   - **Публичный ключ** — используется для шифрования данных.
   - **Приватный ключ** — используется для дешифрования.

2. **Процесс шифрования:**
   - Шифрование в Kyber включает в себя выбор случайного вектора и применение операции на решётке для создания зашифрованного сообщения.
   - Зашифрованное сообщение состоит из нескольких частей, и для его дешифрования нужен приватный ключ.

3. **Процесс дешифрования:**
   - Для дешифрования приватный ключ используется для извлечения исходного сообщения из зашифрованных данных.
   - Процесс дешифрования использует операцию, которая является «обратной» операции шифрования, но с использованием приватного ключа.

4. **Решётки:**
   - Секретность схемы основывается на сложности задачи нахождения точного представления решётки.
   - В Kyber используется структура решётки для создания криптографических функций, которые являются вычислительно сложными для решения даже с использованием квантовых компьютеров.

5. **Безопасность:**
   - Kyber имеет несколько параметров безопасности (например, размер ключа), которые можно настроить в зависимости от требуемого уровня безопасности.
   - Безопасность схемы основана на проблемах, которые считаются сложными даже для квантовых компьютеров, что делает Kyber привлекательным для постквантовых криптографических систем.

Таким образом, основная идея Kyber заключается в использовании математических решёток и сложных операций над ними, чтобы обеспечить высокий уровень безопасности для передачи данных в условиях квантовой вычислительной угрозы.


## Как самой библиотекой пользоватся

Есть по 3 функции на каждый уровень kyber.
1. **Kyber512:**
 - `public_key, secret_key = PythonKyber.Kyber512.generate_keypair()`
 - `ciphertext, shared_secret_enc = PythonKyber.Kyber512.encapsulate(public_key)`
 - `shared_secret_dec = PythonKyber.Kyber512.decapsulate(secret_key, ciphertext)`

2. **Kyber768:**
 - `public_key, secret_key = PythonKyber.Kyber768.generate_keypair()`
 - `ciphertext, shared_secret_enc = PythonKyber.Kyber768.encapsulate(public_key)`
 - `shared_secret_dec = PythonKyber.Kyber768.decapsulate(secret_key, ciphertext)`

3. **Kyber1024:**
 - `public_key, secret_key = PythonKyber.Kyber1024.generate_keypair()`
 - `ciphertext, shared_secret_enc = PythonKyber.Kyber1024.encapsulate(public_key)`
 - `shared_secret_dec = PythonKyber.Kyber1024.decapsulate(secret_key, ciphertext)`


### Нормальный пример:

```
from PythonKyber import Kyber512, Kyber768, Kyber1024

# Пример для Kyber512
public_key_512, secret_key_512 = Kyber512.generate_keypair()
ciphertext_512, shared_secret_encrypted_512 = Kyber512.encapsulate(public_key_512)
shared_secret_decrypted_512 = Kyber512.decapsulate(secret_key_512, ciphertext_512)
assert shared_secret_encrypted_512 == shared_secret_decrypted_512

# Пример для Kyber768
public_key_768, secret_key_768 = Kyber768.generate_keypair()
ciphertext_768, shared_secret_encrypted_768 = Kyber768.encapsulate(public_key_768)
shared_secret_decrypted_768 = Kyber768.decapsulate(secret_key_768, ciphertext_768)
assert shared_secret_encrypted_768 == shared_secret_decrypted_768

# Пример для Kyber1024
public_key_1024, secret_key_1024 = Kyber1024.generate_keypair()
ciphertext_1024, shared_secret_encrypted_1024 = Kyber1024.encapsulate(public_key_1024)
shared_secret_decrypted_1024 = Kyber1024.decapsulate(secret_key_1024, ciphertext_1024)
assert shared_secret_encrypted_1024 == shared_secret_decrypted_1024
```


## Установка

1. Способ через pip:
 - Команда `pip install PythonKyber`
 - Условия:
     - Должен быть `maturin`
     - Должен быть `Rust` и `Cargo` если нет подходящего колеса, pip будет устаналивать из sdist

2. Из исходников:
 - Установить `Rust` и `Cargo`
 - Клонировать репозиторий `git clone https://github.com/kostya2023/PythonKyber.git`
 - Создать venv или другую среду, `python -m venv venv`
 - Установить `maturin`
 - Прописать `maturin build --release`
 - После билдинга `pip install target/wheels/PythonKyber*.whl`

## License

[MIT](https://choosealicense.com/licenses/mit/) См файл LICENSE


## Авторы

- [kostya2023](https://www.github.com/kostya2023)
- [rustpq](https://github.com/rustpq)

