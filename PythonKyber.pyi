from typing import Tuple

class Kyber512:
    """
    Модуль для работы с Kyber512.
    """

    @staticmethod
    def generate_keypair() -> Tuple[bytes, bytes]:
        """
        Генерирует пару ключей (публичный и секретный) для Kyber512.
        Возвращает:
            public_key: Публичный ключ (bytes)
            secret_key: Секретный ключ (bytes)
        """
        ...

    @staticmethod
    def encapsulate(public_key: bytes) -> Tuple[bytes, bytes]:
        """
        Инкапсуляция (шифрование) для Kyber512.
        Аргументы:
            public_key: Публичный ключ (bytes)
        Возвращает:
            ciphertext: Зашифрованное сообщение (bytes)
            shared_secret: Общий секрет (bytes)
        """
        ...

    @staticmethod
    def decapsulate(secret_key: bytes, ciphertext: bytes) -> bytes:
        """
        Декапсуляция (расшифровка) для Kyber512.
        Аргументы:
            secret_key: Секретный ключ (bytes)
            ciphertext: Зашифрованное сообщение (bytes)
        Возвращает:
            shared_secret: Общий секрет (bytes)
        """
        ...


class Kyber768:
    """
    Модуль для работы с Kyber768.
    """

    @staticmethod
    def generate_keypair() -> Tuple[bytes, bytes]:
        """
        Генерирует пару ключей (публичный и секретный) для Kyber768.
        Возвращает:
            public_key: Публичный ключ (bytes)
            secret_key: Секретный ключ (bytes)
        """
        ...

    @staticmethod
    def encapsulate(public_key: bytes) -> Tuple[bytes, bytes]:
        """
        Инкапсуляция (шифрование) для Kyber768.
        Аргументы:
            public_key: Публичный ключ (bytes)
        Возвращает:
            ciphertext: Зашифрованное сообщение (bytes)
            shared_secret: Общий секрет (bytes)
        """
        ...

    @staticmethod
    def decapsulate(secret_key: bytes, ciphertext: bytes) -> bytes:
        """
        Декапсуляция (расшифровка) для Kyber768.
        Аргументы:
            secret_key: Секретный ключ (bytes)
            ciphertext: Зашифрованное сообщение (bytes)
        Возвращает:
            shared_secret: Общий секрет (bytes)
        """
        ...


class Kyber1024:
    """
    Модуль для работы с Kyber1024.
    """

    @staticmethod
    def generate_keypair() -> Tuple[bytes, bytes]:
        """
        Генерирует пару ключей (публичный и секретный) для Kyber1024.
        Возвращает:
            public_key: Публичный ключ (bytes)
            secret_key: Секретный ключ (bytes)
        """
        ...

    @staticmethod
    def encapsulate(public_key: bytes) -> Tuple[bytes, bytes]:
        """
        Инкапсуляция (шифрование) для Kyber1024.
        Аргументы:
            public_key: Публичный ключ (bytes)
        Возвращает:
            ciphertext: Зашифрованное сообщение (bytes)
            shared_secret: Общий секрет (bytes)
        """
        ...

    @staticmethod
    def decapsulate(secret_key: bytes, ciphertext: bytes) -> bytes:
        """
        Декапсуляция (расшифровка) для Kyber1024.
        Аргументы:
            secret_key: Секретный ключ (bytes)
            ciphertext: Зашифрованное сообщение (bytes)
        Возвращает:
            shared_secret: Общий секрет (bytes)
        """
        ...