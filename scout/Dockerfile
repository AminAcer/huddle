FROM python:3.10-slim

WORKDIR /app

COPY pyproject.toml poetry.lock /app/

RUN pip install poetry

RUN poetry config virtualenvs.create false
RUN poetry install

COPY . /app

CMD ["poetry", "run", "scout"]
