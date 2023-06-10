import {
  NumberInputProps as MantineNumberInputProps,
  NumberInput as MantineInput,
} from '@mantine/core';
import { RegisterOptions, useFormContext } from 'react-hook-form';

interface NumberInputProps extends MantineNumberInputProps {
  name: string;
  registerOptions?: RegisterOptions;
}

const NumberInput = (props: NumberInputProps) => {
  const { name, registerOptions, ...otherProps } = props;
  const { register, setValue, formState } = useFormContext();
  const { errors, defaultValues } = formState;
  const { onBlur, name: registName, ref } = register(name, registerOptions);

  const registerRef = (instance: any) => {
    if (instance === null) return;
    ref(instance);
  };

  const handleChange = async (value: number | '') => {
    setValue(name, value);
  };

  return (
    <MantineInput
      my="md"
      {...otherProps}
      error={errors[name]?.message as string}
      name={registName}
      onBlur={onBlur}
      onChange={handleChange}
      ref={registerRef}
      defaultValue={defaultValues ? defaultValues[name] : undefined}
    />
  );
};

export default NumberInput;
