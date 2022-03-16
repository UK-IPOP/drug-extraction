FROM python:3.10

RUN pip install poetry==1.1.13

WORKDIR /python-app

COPY . .

RUN poetry config virtualenvs.create false

RUN poetry install

CMD ["poetry", "run", "python", "-m", "pytest", "--color=auto"]
