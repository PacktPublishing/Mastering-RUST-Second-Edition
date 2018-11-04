
You need a virtual python environment to run the example in this directory.

To install pip and virtualenv run:

```bash
sudo apt-get install python3-pip
sudo pip3 install virtualenv
```

Then, create virtual by running:
```
virtualenv -p /usr/bin/python3.5 test_word_suffix
```

and activate the environment by running
```
source test_word_suffix/bin/activate
```

You should see your prompt change with `(test_word_suffix)` prefix.

After that, Run the example main.py by invoking `python main.py`

To deactivate the virtual environment, run `deactivate`.