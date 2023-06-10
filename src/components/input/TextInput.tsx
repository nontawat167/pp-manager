import {
  TextInput as MantineInput,
  TextInputProps as MantineTextInputProps,
} from '@mantine/core';
import { RegisterOptions, useFormContext } from 'react-hook-form';

export interface TextInputProps extends MantineTextInputProps {
  name: string;
  registerOptions?: RegisterOptions;
}

const TextInput = (props: TextInputProps) => {
  const { name, registerOptions, ...otherProps } = props;
  const { register, formState } = useFormContext();
  const { errors } = formState;

  return (
    <MantineInput
      h={82}
      {...otherProps}
      {...register(name, registerOptions)}
      error={errors[name]?.message as string}
    />
  );
};

export default TextInput;
