import { PropsWithChildren } from 'react';
import { FormProvider, SubmitHandler, useForm } from 'react-hook-form';

type TestFormInput = {
  name: string;
  price: number;
};

type TestFormProviderProps = {
  initValue?: TestFormInput;
  onSubmit: SubmitHandler<TestFormInput>;
};

const TEST_FORM_INPUT_DEFAULT_VALUES = {
  name: '',
  price: 1,
};

const TestFormProvider = ({
  children,
  initValue = TEST_FORM_INPUT_DEFAULT_VALUES,
  onSubmit,
}: PropsWithChildren<TestFormProviderProps>) => {
  const formContext = useForm<TestFormInput>({
    defaultValues: initValue ?? TEST_FORM_INPUT_DEFAULT_VALUES,
    mode: 'onTouched',
  });
  return (
    <FormProvider {...formContext}>
      <form onSubmit={formContext.handleSubmit(onSubmit)} noValidate>
        {children}
      </form>
    </FormProvider>
  );
};

export { TestFormProvider };
export type { TestFormInput };
